#!/usr/bin/env python3
"""
Micro-trainer for incremental online learning
Uses River for streaming ML
"""

import argparse
import json
import sys
import pickle
from pathlib import Path
from typing import Dict, Any
from river import linear_model, optim, compose, preprocessing


class MicroTrainer:
    def __init__(self, model_path: str = "~/.local/share/dad/model.pkl"):
        """Initialize or load existing model"""
        self.model_path = Path(model_path).expanduser()
        self.model_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Create pipeline: normalize → logistic regression
        self.model = compose.Pipeline(
            preprocessing.StandardScaler(),
            linear_model.LogisticRegression(optimizer=optim.SGD(0.01))
        )
        
        # Load existing model if available
        if self.model_path.exists():
            with open(self.model_path, 'rb') as f:
                self.model = pickle.load(f)
    
    def extract_features(self, event: Dict[str, Any]) -> Dict[str, float]:
        """
        Extract numerical features from error event
        
        For now, simple features. Can be expanded with embeddings.
        """
        features = {
            'exit_code': float(event.get('exit_code', 0)),
            'stderr_len': float(len(event.get('stderr', ''))),
            'command_len': float(len(event.get('command', ''))),
            'has_diff': 1.0 if event.get('context_diff') else 0.0,
        }
        
        # Add embedding features if available
        if event.get('embedding'):
            for i, val in enumerate(event['embedding'][:32]):  # Use first 32 dims
                features[f'emb_{i}'] = float(val)
        
        return features
    
    def train_one(self, event: Dict[str, Any], label: bool = True) -> None:
        """
        Perform one incremental training step
        
        label: True if error is "expected to recur", False otherwise
        """
        features = self.extract_features(event)
        
        # Update model with one sample
        self.model.learn_one(features, label)
        
        # Save updated model
        with open(self.model_path, 'wb') as f:
            pickle.dump(self.model, f)
    
    def predict(self, event: Dict[str, Any]) -> float:
        """Predict probability of error recurring"""
        features = self.extract_features(event)
        
        try:
            proba = self.model.predict_proba_one(features)
            return proba.get(True, 0.0)
        except:
            return 0.5  # Neutral if model not ready


def main():
    parser = argparse.ArgumentParser(description="Micro-trainer for error learning")
    parser.add_argument("--event", type=str, help="JSON string of error event")
    parser.add_argument("--file", type=str, help="JSON file with error event")
    parser.add_argument("--label", type=str, default="true",
                       help="Training label (true/false)")
    parser.add_argument("--predict", action="store_true",
                       help="Predict instead of train")
    parser.add_argument("--model-path", type=str,
                       default="~/.local/share/dad/model.pkl",
                       help="Path to model file")
    
    args = parser.parse_args()
    
    trainer = MicroTrainer(model_path=args.model_path)
    
    # Load event
    if args.event:
        event = json.loads(args.event)
    elif args.file:
        with open(args.file, 'r') as f:
            event = json.load(f)
    else:
        event = json.load(sys.stdin)
    
    if args.predict:
        # Prediction mode
        prob = trainer.predict(event)
        print(json.dumps({"probability": prob}))
    else:
        # Training mode
        label = args.label.lower() == "true"
        trainer.train_one(event, label=label)
        print(json.dumps({"status": "trained"}))


if __name__ == "__main__":
    main()

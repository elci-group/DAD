#!/usr/bin/env python3
"""
Embedding generation for error events using SentenceTransformers
"""

import argparse
import json
import sys
from typing import List
import numpy as np
from sentence_transformers import SentenceTransformer


class ErrorEmbedder:
    def __init__(self, model_name: str = "all-MiniLM-L6-v2"):
        """Initialize the embedding model"""
        self.model = SentenceTransformer(model_name)
    
    def embed_event(self, event: dict) -> List[float]:
        """
        Create embedding from error event
        
        Combines command, stderr, and environment signature into a single vector
        """
        # Construct composite text representation
        text_parts = [
            f"Command: {event['command']}",
            f"Error: {event['stderr'][:512]}",  # Limit stderr length
            f"Environment: {event['env_signature']}",
        ]
        
        if event.get('context_diff'):
            text_parts.append(f"Diff: {event['context_diff'][:256]}")
        
        text = " | ".join(text_parts)
        
        # Generate embedding
        embedding = self.model.encode(text, convert_to_numpy=True)
        return embedding.tolist()
    
    def embed_batch(self, events: List[dict]) -> List[List[float]]:
        """Batch embedding for multiple events"""
        texts = []
        for event in events:
            text_parts = [
                f"Command: {event['command']}",
                f"Error: {event['stderr'][:512]}",
                f"Environment: {event['env_signature']}",
            ]
            texts.append(" | ".join(text_parts))
        
        embeddings = self.model.encode(texts, convert_to_numpy=True)
        return embeddings.tolist()


def main():
    parser = argparse.ArgumentParser(description="Generate embeddings for error events")
    parser.add_argument("--event", type=str, help="JSON string of error event")
    parser.add_argument("--file", type=str, help="JSON file with error events")
    parser.add_argument("--model", type=str, default="all-MiniLM-L6-v2",
                       help="SentenceTransformer model name")
    
    args = parser.parse_args()
    
    embedder = ErrorEmbedder(model_name=args.model)
    
    if args.event:
        # Single event from command line
        event = json.loads(args.event)
        embedding = embedder.embed_event(event)
        print(json.dumps(embedding))
    
    elif args.file:
        # Batch from file
        with open(args.file, 'r') as f:
            events = json.load(f)
        
        if isinstance(events, dict):
            events = [events]
        
        embeddings = embedder.embed_batch(events)
        print(json.dumps(embeddings))
    
    else:
        # Read from stdin
        event = json.load(sys.stdin)
        embedding = embedder.embed_event(event)
        print(json.dumps(embedding))


if __name__ == "__main__":
    main()

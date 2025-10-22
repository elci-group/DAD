from setuptools import setup, find_packages

setup(
    name="dad-ml-sidecar",
    version="0.1.0",
    description="Machine learning sidecar for DAD (Dumbass Daemon)",
    author="DAD Project Contributors",
    packages=find_packages(),
    python_requires=">=3.9",
    install_requires=[
        "numpy>=1.24.0",
        "sentence-transformers>=2.2.0",
        "river>=0.20.0",
        "duckdb>=0.9.0",
        "fasttext-wheel>=0.9.2",
    ],
    entry_points={
        "console_scripts": [
            "dad-train=ml_sidecar.train:main",
            "dad-embed=ml_sidecar.embed:main",
        ],
    },
)

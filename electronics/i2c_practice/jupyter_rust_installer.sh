#!/bin/bash

# Install Jupyter Notebook for Rust

# Create and activate a virtual environment
python -m venv rust_jupyter_env

if [[ "$OSTYPE" == "msys" ]]; then
    source rust_jupyter_env/Scripts/activate
else
    source rust_jupyter_env/bin/activate
fi

echo "Installing Jupyter Notebook in virtual environment $VIRTUAL_ENV"

# Install Jupyter Notebook
pip install jupyter
pip install cmake
pip install jupyterlab
pip install nodejs

# Install Rust Kernel
cargo install --locked evcxr_jupyter
evcxr_jupyter --install

jupyter labextension install jupyterlab-plotly
jupyter labextension install @jupyter-widgets/jupyterlab-manager plotlywidget
jupyter labextension install @jupyterlab/plotly-extension

# Install Monokai theme
pip install jupyterthemes
jt -t monokai

# Start Jupyter Notebook
jupyter lab
#jupyter notebook




# Rust Notebook environment

This is a Jupyter notebook environment for Rust, based on the `[evcxr]`
kernel. It is intended to be used with the `[JupyterLab]` interface.

1. Install the following libraries 
    ```requirements
    jupyter
    cmake
    jupyterlab
    nodejs
    ```
2. Install the `evcxr` kernel
    ```shell
    cargo install evcxr_jupyter
    evcxr_jupyter --install
    ```
3. Install Jupyter extensions
    ```shell
   jupyter labextension install jupyterlab-plotly
   jupyter labextension install @jupyter-widgets/jupyterlab-manager plotlywidget
   jupyter labextension install @jupyterlab/plotly-extension
    ```
   
4. Customize Jupyterlab **optional**
   ```requirements
    jupyterthemes
   ```
   ```shell
   pip install jupyterthemes
   jt -t monokai
   ```
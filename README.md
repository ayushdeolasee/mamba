# mamba

## CLI commands:
- mamba create [project_name] [env name if special else default to name of project]
- mamba install [package_name/ if none provided then look at py.toml file and install all packages]
- mamba create env [name_of_env]
- mamba delete env [name_of_env]
- mamba template [name_of_template]
- mamba uninstall [name_of_package]

## Mamab Create
- Creates a new folder with project_name
- Creates a main.py file, a new conada env and automatically activates the env, and a py.toml file.

## Todo:
[] Create a basic cli in rust that will compile to run a command mamba
[] Plan out toml file structure 
[] Create the mamba create command
[] Create the mamba install command
[] Create the mamba create/delete env commands 
[] Create the mamba template command
[] Create the mamba uninstall command
[] Figure out virtual env requirements (conda for now)
[] Figure out how to add a new package to file and install using pip 
[] Create preset project structures (for ML which will install pytorch, pandas, numpy, and matplotlib already)

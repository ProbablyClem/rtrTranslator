# A tool to create translation files for the [rtr](https://github.com/ProbablyClem/RustTraduction) librairy


## Instruction :

Launch the app and then enter commands.

### Commands : 
- view [path]<br/>
    Will show you every txt files in the folder in argument. (./lang/ by default)
- create [path] [filename]<br/>
    Will create a new file with every .rs files in the folder. The file will be called [filename].txt (default name "origin") (Current folder by default)
    Use "." to select the current directory
    the file will be create in the ./lang directory
- quit<br/> 
    Exit the app

### How to use it : 

- go to the source directory

- if the origin file does not exist create it with the "$create" command

- create new langage from the origin file with the "$create" command

## Installation
Install with cargo : 
```
cargo install rtr_translator
```

## TODO (as ordered by priority) : 

- access precedent command with the up arrow (as in the terminal)

- autocompletion

- GUI

A tool to create translation files for the rtr librairy (https://github.com/ProbablyClem/RustTraduction)

Instruction :

Launch the app and then enter commands.
Note that only absolut paths works right now
Commands : 
- view [path]
    Will show you every txt files in the folder in argument. (Current folder by default)
- create [path] [filename]
    Will create a new file with every .rs files in the folder. The name will be called [filename].txt (default name "origin") (Current folder by default)
    Use "." to select the current directory
    the file will be create in the ./lang directory
- quit 
    Exit the app

How to use it : 

- go to the source directory

- if the origin file does not exist create it with the "$create" command

- create new langage from the origin file with the "$create" command

TODO (as ordered by priority) : 

- Error handling when the origin file does not exist

- Error handling if the specified path does not exist

- Use of relative paths

- acces precedent command with the up arrow (as in the terminal)

- autocompletion

- GUI

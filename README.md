# rust-learning

```
                                $$\           $$\                                         $$\                     
                                $$ |          $$ |                                        \__|                    
 $$$$$$\  $$\   $$\  $$$$$$$\ $$$$$$\         $$ | $$$$$$\   $$$$$$\   $$$$$$\  $$$$$$$\  $$\ $$$$$$$\   $$$$$$\  
$$  __$$\ $$ |  $$ |$$  _____|\_$$  _|$$$$$$\ $$ |$$  __$$\  \____$$\ $$  __$$\ $$  __$$\ $$ |$$  __$$\ $$  __$$\ 
$$ |  \__|$$ |  $$ |\$$$$$$\    $$ |  \______|$$ |$$$$$$$$ | $$$$$$$ |$$ |  \__|$$ |  $$ |$$ |$$ |  $$ |$$ /  $$ |
$$ |      $$ |  $$ | \____$$\   $$ |$$\       $$ |$$   ____|$$  __$$ |$$ |      $$ |  $$ |$$ |$$ |  $$ |$$ |  $$ |
$$ |      \$$$$$$  |$$$$$$$  |  \$$$$  |      $$ |\$$$$$$$\ \$$$$$$$ |$$ |      $$ |  $$ |$$ |$$ |  $$ |\$$$$$$$ |
\__|       \______/ \_______/    \____/       \__| \_______| \_______|\__|      \__|  \__|\__|\__|  \__| \____$$ |
                                                                                                        $$\   $$ |
                                                                                                        \$$$$$$  |
                                                                                                         \______/ 
```

## Yet another learning project.

This one focuses specifically on coding exercises from [The Rust Book](https://doc.rust-lang.org/book/) and [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/).

I'm also following the [Learning Rust](https://www.youtube.com/playlist?list=PLrmY5pVcnuE_dyWibakRuGJcuiwAkhGZB) playlist by [Brooks Builds](https://www.youtube.com/channel/UCT1-XRVnJA-wws2bfbLbFcQ) on YouTube, as he follows through The Rust Book.

## Source Directories

0. src.blank - this is a boilerplate *Hello, World!* source folder which I duplicate and rename whenever I need a fresh **src** folder.
1. src - current working source directory, assume this is work-in-progress.
2. src.chap2 - the *Guessing Game* exercise from Chapter 2 of TRB.
3. src.chap2a - a reverse of the *Guessing Game* from Chapter 2 of RTB, using the rand library to guess a user inputted secret value.

## Current Work & Future Plans

The current **src** folder is a revised version of my work on the reverse guessing game, I intend to stop the *rand* library from guessing the same incorrect values multiple times so a maximum of 99 incorrect guesses can be made before the program gets the correct value. After this I will be working on my own guessing algorithm based around whether a guessed values is greater or less than the secret number provided by the user.
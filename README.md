# dewabbit
A De-Duplicator program written in rust. 

This deduplicator only goes one deep (for now), primarily because that is all I needed it to do. It also does not look at the extension of files. So it will hash every file within a chosen directory. 

## COMPAT
This is only supported on windows(for now), because that is what I need it to do. 

## Getting started

Simply choose a directory to dedupe (select a folder) 

![image](https://user-images.githubusercontent.com/58314490/221382909-d520045b-0f77-4fcc-a0eb-8ef0d551fd85.png)

Then select "Go!" and the deduplicator will roll through the directory and try to match the contents of each file. If you get a match, a directory is given to you inside the where you are searching- called "dupes" 

When the deduplicator is finished it will display "All Done," when the dialog is closed- the program will exit. 

![image](https://user-images.githubusercontent.com/58314490/221382943-0a7208a5-d345-4817-8422-ad31e7302ecd.png)

Cheers,
-nate

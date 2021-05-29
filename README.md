Prettify
========

The goal of this utility is to create a generic command line program to prettify text.
The initial version will handle JSON and XML, with the capability to add more later.
If there is a format that is requested on GitHub, I will consider adding it.

I wanted to create this project for three reasons:

The first is that I have never found a simple solution to do this function except going to a converter website, but I don't feel comfortable pasting random data to a site I don't trust.
It isn't hard to prettify text, and I was surprised I couldn't find a simple command line program that could handle multiple types of inputs.

The second reason I wanted to create this project is that I want to improve my Rust skills.
I taught myself the language recently, but haven't found a good project to continue honing my skills till now.

Finally, I want to learn how to actually ship an application.
All of my previous projects require someone to compile my code from source, and I want to learn how packaging works.
My goal is that when I am done, I will publish my application on Crates, Snap, and Apt to start with.
In time, I may try adding Yum, Flatpak, AppImage, and even Brew, but I want to start with a smaller target to begin with.
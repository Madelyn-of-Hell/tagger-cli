# Log 1: Basic implementation of tagging system

This is kind of big for my first log, because I wasn't initially planning on putting this on stardance.

Essentially, a few years ago I came across a video I'm not going to link where the youtuber detailed their troubled search for a good tag-based filesystem, before ultimately resorting to implementing their own. I thought that seemed pretty cool, and despite the fact I've never used nor felt a need for a tag-based system, I downloaded the project to try it out.

I was immediately struck by two things:
1. The released binaries wouldn't launch, throwing numerous errors. When I eventually solved them all manually (expected packages not being present etc.), the app ran slowly and painfully.
2. The project. Was written. Entirely. In python.

Look. There's nothing wrong with python. It's a useful language with many valid applications.
At the same time.

***REALLY?***
This is the best we can do?

IDK there was something that struck me as so lackluster about it that my immediate reaction was *'I could do better than that.'*
So here I now am to do just that.

The lightbulb moment that brought me back to the project a week or so ago was that I could probably implement the whole thing with directories and symlinks.
Basically, if I create a directory tree in which each directory is a tag and each file that has a symlink in that directory has that tag, then I have the capacity to apply as many tags as I want to a given file, I have the ability to nest tags, and searching these tags should be as easy as navigating a typical filesystem.
At a very basic level, this first log contains all those functions. You can add tags, remove tags, tag files, and search the tags, though each one of these functions has issues.
Things I plan to resolve in the future include, but are not limited to:

Tag adding/management:
- Multiple inheritance for child tags. Should be able to do this with symlinks.
- Finding a way to detect tagged files moving. Currently, this breaks my symlinks, invalidating the whole process. One possible solution is removing the files from the user: locking them in a directory that can't be modified. This sucks and is bad because it means that you have to surrender a classical filing system to use the tag-based one. Ideally, this system should be a neat addition to the system.
Searching:
- Nested tags should be searchable without using the full filepath from the base tag for convenience. This brings up the potential for collisions; I should be able to handle this somehow. Short of a full scan of the directory, the only way I can think of to do this is to keep a log of the directory structure, which I prefer not to do because it defeats the point of it being directory by default.
- There should be a command to list all current tags.

There are also other bugs and behaviours I need to fix/improve. This project is also an experiment with test driven development for me, something I've heard much about yet not really played with much. Currently, all tests pass; I expect that to change soon.
I wanna make my code a lot less ugly rn too because I don't like it rn.

Once the CLI is in a stable state, I plan on implementing a UI in iced-rs, which will really just be a frontend that calls the CLI. (Also considering figuring out how to rewrite the CLI as a library that I can use directly, but CLI functionality is a priority.)

uuh yeah that's it for now don't forget to like and subscribe for more epic tagging content.

Also if anyone actually thinks a tagging system would be useful for them please reach out with suggestions because I doubt im ever actually gonna *use* this system lol it just seemed fun to make.
okibaii

# piston-falling-sand-prototype

A simple prototype of a [falling-sand game](https://en.wikipedia.org/wiki/Falling-sand_game), with only 2 types of particles:  
moving sand and immobile obstacle.

**Space** to pause/unpause the game.  
**R** to toggle rain of sand.  
**D** to print debug information to standard output  
**Left** mouse button to draw obstacles.  
**Right** mouse button to delete obstacles and sand.  

Change WINDOW_HEIGHT and WINDOW_WIDTH const to adjust the size of the game screen.  
Change RAIN_SPARSENESS const to change the density of the falling sand.  

This program might require installation of freetype and SDL2 libraries to work.

Compiles with: rustc 1.12.1 (d4f39402a 2016-10-19)

![Screenshot](screenshot.png?raw=true "Screenshot")

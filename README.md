# returnal

*returnal*, the text editor

this is the server. available clients:

* [returnal-term](https://github.com/andrewsuzuki/returnal-term)

## goals

* primarily targeting modern platforms, not the amiga
* modal editing (a la vim)
* core plugins providing expected functionality (like atom)
* server / client architecture with communication via websockets, allowing (1) multiple frontends (terminal, web, gui) for the same core and user configuration, (2) super quick startup, regardless of plugin load (3) non-hacky pair programming, and (4) much more
* react-like view composition for efficient updates sent through said websockets
* async jobs
* scriptable in something timeless and fast (probably [lisp](https://en.wikipedia.org/wiki/Greenspun%27s_tenth_rule), a la emacs)
* command palette (a la sublime/atom)
* plugin manager

## default plugins

* super quick fuzzy file finder
* tree view
* sensible language support (a la atom)
* a few color schemes
* editorconfig

## roadmap

* server setup
* server-side view functionality (components/boxes, re-render tree, etc)
* terminal client
* text buffers
* buffer -> view
* define modes
* cursor (single, selection, multiples of either)
* naive insert mode
* keyboard command queue
* basic cursor movement commands (j/k/l/h, gg, G, C-d/u/e/y/b/f, 0, $, ! (^), w/b/e)
* objects
* status bar
* command line
* writing to file
* opening from file
* basic :-commands (w, q, e)
* line numbers (relative or static)
* ... more commands and objects ...
* search
* syntax highlighting
* color schemes

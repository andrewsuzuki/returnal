# notes

general notes for *returnal*

## key objects

**returnal**

**editor**

**client**

**buffer**

**mode**

**cursor**

**selection**

**view**

**command** - something to be done. in normal mode, consists of [?count][?operator][?count][?motion]. in visual mode, consists of just [?count][operator] (the selection takes place of the motion)

**operator** - enum: either a *BufferAction* (ex. insertion, deletion, replacement, undo, redo, indent) or an *EditorAction* (ex. save, exit, set mark, set mode, new buffer, window movement/resizing/opening/closing, etc). is scrolling editor or buffer?

**motion** - enum: either a *Movement* (ex. h/j/k/l, w, b, $, 0, f[c], t[c], }, e) or a *Gathering* (prefaced by a or i, ex. aw, ip)

## relationships

**main -> returnal[one]**

**returnal -> editor[many]** uses: a "cloud" version of returnal with isolated editor instances for multiple clients

**editor -> client[many]** uses: pair programming or teaching

**editor -> buffer[many]** similar to vim

**editor -> mode** carries across buffers

**editor -> view** the "root" view, with recursively-defined subviews

**editor -> register[a-z+]** global registers

**client -> sender** websocket sender

**client -> register[a-z+]** client registers

**buffer -> type** such as normal file, read-only, help, status, overlay

**buffer -> content** the actual text content, represented by a rope probably

**buffer -> cursor[many]** multi-cursor support -- each belongs to a single client

**buffer -> selection[many]** multi-select support -- each belongs to a single client

**mode -> coseq trie** turns letter sequences into commands

**view -> view[many]** recursive subviews, with associated bin-packed positions and dimensions

**view -> dimensions** box size

**view -> position** position relative to parent

### normal mode

'i', 'I', 'a', 'A', 'v', 'V', 'ctrl+v', ':', '/', '?', etc. switch to respective modes (insert, visual, visual line, visual block, command line)

tentative rules for completing commands (taken from iota):

```
|-action------------|-number-|-object-|-reference----|-kind-|-result------------------------------------------------------------|
| n                 | n      | n      | n            | y    | move cursor to next (default) text object with kind               |
| n                 | n      | n      | y            | -    | move cursor to text object with reference + default anchor        |
| n                 | n      | y      | -            | -    | move cursor to text object                                        |
| n                 | y      | n      | n            | y    | move cursor to nth instance of kind (from start of buffer)        |
| n                 | y      | n      | y   (index)  | -    | set index to number, cursor to ref + default anchor               |
| n                 | y      | n      | y   (offset) | -    | multiply offset by number, cursor to ref + default anchor         |
| y   (instruction) | -      | -      | -            | -    | send instruction to editor with whatever parameters are available |
| y   (operation)   | n      | n      | n            | y    | apply operation to kind at cursor (default anchor)                |
| y   (operation)   | n      | n      | y            | -    | apply operation to reference with default anchor                  |
| y   (operation)   | n      | y      | -            | -    | apply operation to object                                         |
| y   (operation)   | y      | n      | n            | y    | apply operation to nth instance of kind                           |
```

### command line mode

':', '/', '?' all go into command line mode. the command line will be prefaced with the character.

if the line becomes empty, close and return to normal mode

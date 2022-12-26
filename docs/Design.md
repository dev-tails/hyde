# Design

**hyde** is a web site builder.  It takes some of its starting principles from [jekyll](https://jekyllrb.com/), but improves upon them.

## Minimum Viable Product

The MVP of hyde is a single executable that serves the static webpage and allows the author to add new posts.  

The first iteration of the server should render an HTML home page that has links to the individual posts.  Clicking on a link to a post should open up that post at a URL that matches a text file on the file system.  Initially creating/editing of posts will be done via a text editor on the computer. 
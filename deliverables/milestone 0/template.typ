// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(title: "", authors: (), date: none, repo: "", website: "", body) = {
  // Set the document's basic properties.
  set document(author: authors, title: title)
  set page(numbering: "1", number-align: center)
  set heading(numbering: "1.")
  set text(font: "GFS Neohellenic", lang: "en")

  show link: it => {
    set text(fill: purple)
    it
  }

  // Title row.
  align(center)[
    #block(text(weight: 700, 1.75em, title))
    #v(1em, weak: true)
    #link(repo)[#underline(text(weight: 400, 1.3em, "Git Repository"))]
    #h(1em)
    #link(website)[#underline(text(weight: 400, 1.3em, "ebb.csclub.cloud"))]
  ]

  // Main body.
  set par(justify: true)
  set text(hyphenate: false)

  body
}
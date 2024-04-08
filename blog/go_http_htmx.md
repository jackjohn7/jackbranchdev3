---
title: Go+HTMX+Tailwind
url: go_htmx_tailwind
description: Creating full-stack applications with Go, HTMX, and TailwindCSS using net/http and Templ
date: 04-07-2024
date_updated: 04-07-2024
public: true
---

# Introduction

With the recent improvements to net/http as of the Go 1.22
update, I was excited to build an application with no
external dependencies for the web server.

I wouldn't necessarily recommend this to everyone though. 
Ensuring that errors are properly bubbled up through 
middleware with standard `http.HandlerFunc` chaining is
not the simplest thing in the world and I didn't 
particularly enjoy that. Frameworks like 
<a>Echo</a>, <a>Gin</a>, and <a>Chi</a> all make that a
much better experience since their handlers are built to
return errors.

That said, if you're set on sticking to the standard
library, you can build a very capable web application 
with less boilerplate code than you could in the past.

# Why Go 1.22?

If you're unaware, Go 1.22 provides improvements to route 
handlers since you can now specify what HTTP method the 
route is meant to handle.

Go 1.21

```go
http.HandleFunc("/todos", func (w http.ResponseWriter, r *http.Request) {
  switch r.Method {
  case http.MethodGet:
    // respond with some todos in HTML
  case http.MethodPost:
    // respond with the new todo in HTML
  default:
    // return MethodNotAllowed
  }
})
```

Go 1.22

```go
http.HandleFunc("GET /todos", func (w http.ResponseWriter, r *http.Request) { /* Your GET func */} )
http.HandleFunc("POST /todos", func (w http.ResponseWriter, r *http.Request) { /* Your POST func */} )

// unsupported methods like DELETE, PUT, and PATCH will get MethodNotAllowed error response
```

The code in Go 1.22 is a lot simpler as I think you'll
agree.

# Templates

For my project, I chose to use
<a>Templ</a>,
but you can absolutely get away with using the standard
`html/template` templating library. Both just allow us to
create dynamic, server-rendered HTML with ease. The 
advantage of Templ is that you get compile-time type-safety
and a powerful syntax for generating your HTML.


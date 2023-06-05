# How I Learn

This repository contain amount of free learning resource which I read in high school.

- [How I Learn](#how-i-learn)
  - [💻 Programming](#-programming)
  - [🧮 Math](#-math)
  - [🌏 Science](#-science)

## 💻 Programming

- 🎬 Video
  {% for item in programming.video %}
  - {{item.name}} [↗️]({{item.url}}) [🏠](#how-i-learn)
  
    {{item.description}}
  {% endfor %}

- 📄 Article
  {% for item in programming.article %}
  - {{item.name}} [↗️]({{item.url}}) [🏠](#how-i-learn)
  
    {{item.description}}
  {% endfor %}

## 🧮 Math

- 🎬 Video
  {% for item in math.video %}
  - {{item.name}} [↗️]({{item.url}}) [🏠](#how-i-learn)

    {{item.description}}
  {% endfor %}

- 📄 Article
  {% for item in math.article %}
  - {{item.name}} [↗️]({{item.url}}) [🏠](#how-i-learn)
  
    {{item.description}}
  {% endfor %}

## 🌏 Science

- 🎬 Video
  {% for item in science.video %}
  - {{item.name}} [↗️]({{item.url}}) [🏠](#how-i-learn)
  
    {{item.description}}
  {% endfor %}

- 📄 Article
  {% for item in science.article %}
  - {{item.name}} [↗️]({{item.url}}) [🏠](#how-i-learn)
  
    {{item.description}}
  {% endfor %}

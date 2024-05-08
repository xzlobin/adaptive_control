---
title: Welcome
layout: default
---

## Formulácia problému

Je potrebné navrhnúť adaptívny riadiaci systém pre riadenie kurzu nákladnej lode, ktorého základom je Lyapunovova teória stability. 

Systém:
$$
\begin{equation}
\dddot{\varphi}(t)+\left(\frac{\tau_{1}+\tau_{2}}{\tau_{1}\tau_{2}}\right)\ddot{\varphi}(t)+\left(\frac{1}{\tau_{1}\tau_{2}}\right)\dot{\varphi}(t)=\frac{K}{\tau_{1}\tau_{2}}\left(\tau_{3}\dot{u}(t)+u(t)\right),    
\label{main_sys}
\end{equation}
$$
kde $\varphi(t)$ [rad] je uhol natočenia lode a táto veličina je výstupnou
veličinou riadeného systému. Vstupom je uhol vychýlenia kormidla $u(t)$ [rad]. 
Referenčný model pre URO je v tvare
\begin{equation*}
W_{m}(s)=\frac{k_{m}\cdot Z_{m}(s)}{R_{m}(s)}=\frac{\overbrace{\frac{1}{T_{m}^{2}}}^{k_{m}}\cdot\overbrace{1}^{Z_{m}(s)}}{\underbrace{\left(s+\frac{1}{T_{m}}\right)^{2}}_{R_{m}(s)}},
\end{equation*}
kde $k_m$ je vysokofrekvenčné zosilnenie referenčného modelu, $Z_m(s)$ je
monický Hurwitzov polynóm stupňa 0, $R_m(s)$ je monický Hurwitzov polynóm stupňa
2, pričom relatívny stupeň $n^\star_m=2$.

Welcome to markdown madness. We hope you **really** enjoy using good old text for writing.

Just type some [markdown](http://en.wikipedia.org/wiki/Markdown)
and jekyll will automatically turn it into hypertext markup language (HTML). *Simple as that.*

> Quote goes here.

A list:

- One
- Two
- Three

Some inline code `to_html` and a preformatted code block:

```
Kramdown::Document.new( 'Hello Markdown!' ).to_html
```

with code highlighting:

``` ruby
# The Greeter class

class Greeter
  def initialize(name)
    @name = name.capitalize
  end

  def salute
    puts "Hello #{@name}!"
  end
end

# Create a new object
g = Greeter.new("world")

# Output "Hello World!"
g.salute
```

Testing MathJax Formulas

$$ f(x) = \frac{1}{\sigma \sqrt{2\pi} } e^{-\frac{1}{2}\left(\frac{x-\mu}{\sigma}\right)^2} $$

Testing Plotly figures

{% include figures/test.html %}

Or try

# Heading 1

## Heading 2

### Heading 3
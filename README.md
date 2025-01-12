# mdbook-echarts
A preprocessor and cli tool for mdbook to show chart use [Apache Echarts](https://echarts.apache.org/).

> This repo is in the works-for-me state and I use it inside my nixos configuration to have echarts inside a mdbook.
> The changes are related to bumping the dependencies up and doing some minor updates to the code requied for the package
> to be built (+ commented out tests).
>
> It has been created bacause the original mdbook-echarts are built against old version of mdbook which had borrow checker
> error around handlebar code (fixed ages ago). 
> 
> You should consider it as unmaintained (until I decide otherwise because of whatever reason). Any issue or pr will likely be
> ignored.

#### install

Add the

```
(
  pkgs.rustPlatform.buildRustPackage rec {
    pname = "mdbook-echarts";
    version = "marek-hydra-2025-01-10";

    src = pkgs.fetchFromGitHub {
      owner = "mskorkowski";
      repo = pname;
      rev = "dbb61364b8b9e4b977e97ffc2f89715507fd24cb";
      sha256 = "sha256-ZGLrCOG2ca1iJNfZoO9mqubkw8Hw4RDlQAtq0DuoXuY=";
    };

    cargoSha256 = "sha256-CdGG7DwSoYvN6TJYn+uM9KP2Nr5gJBYxA4CzMec6384=";

    meta = with pkgs.lib; {
      description = "A preprocessor and cli tool for mdbook to show chart use Apache Echarts.";
      homepage = "https://github.com/zhuangbiaowei/mdbook-echarts";
      license = licenses.mit;
    };
  }
)
```

In your `home.packages` in the home manager configuration.

#### 1. Use as mdbook preprocessor.

```bash
#cat /path/to/your/mdbook/book.toml

[book]
authors = []
language = "en"
multilingual = false
src = "src"

[build]
create-missing = false

#use as mdbook preprocessor
[preprocessor.echarts]

[output.html]
additional-js = ["assets/echarts.min.js"]

[output.html.fold]
enable = true
level = 0

```

### 2. edit your markdown file
````text

```echarts
{
  xAxis: {
    data: ['A', 'B', 'C', 'D', 'E']
  },
  yAxis: {},
  series: [
    {
      data: [10, 22, 28, 43, 49],
      type: 'bar',
      stack: 'x'
    },
    {
      data: [5, 4, 3, 5, 10],
      type: 'bar',
      stack: 'x'
    }
  ]
};
```

or 

{% echarts %}
{
  xAxis: {
    data: ['A', 'B', 'C', 'D', 'E']
  },
  yAxis: {},
  series: [
    {
      data: [10, 22, 28, 43, 49],
      type: 'bar',
      stack: 'x'
    },
    {
      data: [5, 4, 3, 5, 10],
      type: 'bar',
      stack: 'x'
    }
  ]
};
{% endchart %}

````


When you run 
```bash
mdbook serve
```
Or
```bash
mdbook build
```
this will do something to make sure your chart show as you wish.


![demo](./demo.png)


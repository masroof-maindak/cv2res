#import "@preview/simple-technical-resume:0.1.1": *

#let name = "{{ name }}"
#let phone = "{{ phone }}"
#let email = "{{ email }}"

#let github = "{{ github }}"
#let personal-site = "{{ site }}"
#let linkedin = "{{ linkedin }}"

#show link: set text(fill: blue)
#show link: underline

#show: resume.with(
  top-margin: 0.3in,
  personal-info-font-size: 9.6pt,
  author-position: center,
  personal-info-position: center,
  author-name: name,
  phone: phone,
  email: email,
  website: personal-site,
  // linkedin-user-id: linkedin,
  github-username: github,
)

#custom-title("Projects")[
{% for proj in projects %}
  #project-heading(
    "{{ proj.title }}",
    stack: "{{ proj.stack }}",
    project-url: "{{ proj.url }}"
  )[
{%- for line in proj.desc %}
  - {{ line }}
{%- endfor %}
  ]

{% endfor -%}
]

#custom-title("Smaller Projects/Tools")[
{% for proj in small_projects %}
  #project-heading(
    "{{ proj.title }}",
    project-url: "{{ proj.url }}"
  )[
{%- for line in proj.desc %}
  - {{ line }}
{%- endfor %}
  ]

{% endfor -%}
]

#custom-title("Hobby Projects")[
{% for proj in hobby_projects %}
  #project-heading(
    "{{ proj.title }}",
    project-url: "{{ proj.url }}"
  )[
{%- for line in proj.desc %}
  - {{ line }}
{%- endfor %}
  ]

{% endfor -%}
]

# {{ date }}


## Daily Notes


{% for entry in entries %}
### {{ entry.prompt.name }}

#### {{ entry.prompt.prompt }}

{{ entry.user_entry }}
{% endfor %}
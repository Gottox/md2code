#!/bin/sh -e
{{- range .include }}
. "{{ . }}"
{{- end }}

################################################################################
{{ range .segments -}}
{{   $line := .line -}}
{{   with .code -}}
line_{{$line}}() {
{{      prepend "\t" . -}}
}
{{   end -}}
{{   with .text -}}
{{     prepend "# " . -}}
{{   end -}}
{{ end -}}
################################################################################
{{- range .segments -}}
{{   $line := .line -}}
{{-   with .code }}
line_{{ $line }};
{{-   end }}
{{- end }}

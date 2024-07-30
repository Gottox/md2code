{{ range .include -}}
#include <{{ . }}>
{{ end -}}

{{ "" }}
////////////////////////////////////////////////////////////////////////////////
{{ range  .segments -}}
{{   $line := .line -}}
{{   with .code -}}
static void line_{{$line}}(void) {
{{     prepend "\t" . -}}
}
{{   end -}}
{{   with .text -}}
{{     prepend "// " . -}}
{{   end -}}
{{ end -}}
////////////////////////////////////////////////////////////////////////////////
int main(void) {
{{- range .segments -}}
{{   $line := .line -}}
{{-   with .code }}
	line_{{$line}}();
{{-   end }}
{{- end }}
	return 0;
}

{{/*
Expand the name of the chart.
*/}}
{{- define "opendcs.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" | lower}}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "opendcs.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" | lower}}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" | lower}}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" | lower }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "opendcs.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "opendcs.labels" -}}
helm.sh/chart: {{ include "opendcs.chart" . }}
{{ include "opendcs.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "opendcs.selectorLabels" -}}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "opendcs.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "opendcs.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Return the name for the database to use
*/}}
{{- define "database.name" -}}
  {{- if .Values.postgresql.enabled }}
    {{- printf "%s" (include "postgresql.v1.database" .Subcharts.postgresql) -}}
  {{- else -}}
    {{- printf "%s" (tpl .Values.externalDatabase.database .) -}}
  {{- end -}}
{{- end -}}

{{/*
Return the hostname of the database to use
*/}}
{{- define "database.hostname" -}}
  {{- if .Values.postgresql.enabled -}}
    {{- printf "%s" (include "postgresql.v1.primary.fullname" .Subcharts.postgresql) -}}
  {{- else -}}
    {{- printf "%s" (tpl .Values.externalDatabase.hostname $) -}}
  {{- end -}}
{{- end -}}

{{/*
Get the user-password key for the database password
*/}}
{{- define "database.passwordKey" -}}
  {{- if .Values.postgresql.enabled -}}
    {{- printf "%s" (include "postgresql.v1.userPasswordKey" .Subcharts.postgresql) -}}
  {{- else if .Values.externalDatabase.userPasswordKey -}}
    {{- printf "%s" (tpl .Values.externalDatabase.userPasswordKey $) -}}
  {{- else -}}
    {{- "password" -}}
  {{- end -}}
{{- end -}}


{{/*
Get the name of the secret containing the database password .
*/}}
{{- define "database.secretName" -}}
  {{- if .Values.postgresql.enabled -}}
    {{- printf "%s" (include "postgresql.v1.secretName" .Subcharts.postgresql) -}}
  {{- else if .Values.externalDatabase.existingSecret }}
    {{- printf "%s" (tpl .Values.externalDatabase.existingSecret $) -}}
  {{- else -}}
    {{- printf "%s" (include "mychart.fullname" .) -}}
  {{- end -}}
{{- end -}}

{{/*
Return the name for the user to use
*/}}
{{- define "database.username" -}}
  {{- if .Values.postgresql.enabled -}}
    {{- printf "%s" (include "postgresql.v1.username" .Subcharts.postgresql) -}}
  {{- else -}}
    {{- printf "%s" (tpl .Values.externalDatabase.username $) -}}
  {{- end -}}
{{- end -}}
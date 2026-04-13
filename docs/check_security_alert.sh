#!/bin/sh

gh api   -H "Accept: application/vnd.github+json"   -H "X-Github-Api-Version: 2026-03-10" /repos/kotobukid/axx/dependabot/alerts?state=open  | jq '.[] | {package: .dependency.package.name, severity: .security_advisory.severity}'

---
description: LucidCursor
---

1. Kill any running instances of the app
// turbo
taskkill /F /IM LucidCursor.exe /T 2>&1 | Out-Null
taskkill /F /IM lucid-cursor.exe /T 2>&1 | Out-Null
taskkill /F /IM tauri-applucid-cursor.exe /T 2>&1 | Out-Null

2. Start the development server
// turbo
npm run tauri dev

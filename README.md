# 🍅 Netupi23 - Minimalist Time Tracker & Pomodoro Timer

[**English**](#english) | [**Español**](#español)

---

## English

**Inspired by and forked from the work of [4DA](https://github.com/4DA)**  
*All acknowledgments to 4DA for the original concept and inspiration that made this project possible.*

A minimalist time tracking application written in Rust. Perfect for developers, freelancers, and anyone who wants to track their work time efficiently.

### ✨ Current Features

- **⏰ Pomodoro Timer**: 25-minute focused work sessions 
- **☕ Break Timer**: 5-minute break sessions
- **📊 Project Time Tracking**: Track time across different projects with persistent storage
- **💾 Local Data Storage**: All sessions saved locally in JSON format
- **🎯 Real-time Timer Display**: See your timer progress updating live in the terminal
- **🚀 Interactive Mode**: Intuitive command-line interface with history and auto-completion
- **⚡ Auto-stop Feature**: Starting a new timer automatically stops the current one
- **🔒 Privacy First**: All data stays on your computer - no cloud, no tracking

### 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/gatzer/netupi23.git
cd netupi23

# Build and run
cargo run
```

### 📋 Available Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `work <project-name>` | `w` | Start tracking work time for a specific project |
| `pomodoro` | `pomo` | Start a 25-minute Pomodoro work session |
| `break` | `br` | Start a 5-minute break timer |
| `stop` | `s` | Stop the current timer and save the session |
| `projects` | `proj` | List all projects with total time worked |
| `help` | `h`, `?` | Show available commands |
| `clear` | `cls` | Clear the terminal screen |
| `exit` | `quit`, `q` | Exit the application |

### 💡 Usage Examples

#### Track Work Time
```bash
netupi> work "Website Redesign"
🚀 Starting work session for project: Website Redesign
⏰ Work timer started! Use 'stop' to finish.
   ⚡ Running: 00:15
netupi> stop
✅ Timer stopped!
⏱️  Total time: 15 minutes
💾 Session saved successfully.
```

#### Pomodoro Workflow
```bash
netupi> pomodoro
🍅 Starting 25-minute Pomodoro session...
⏰ Pomodoro started! Use 'stop' to finish early.
   🍅 Running: 00:05

# Timer updates in real-time as you work...
```

#### Auto-stop Feature
```bash
netupi> pomodoro
🍅 Starting 25-minute Pomodoro session...
   🍅 Running: 00:05

netupi> break
⏹️  Previous timer stopped early!
⏱️  Time recorded: 5 minutes
💾 Session saved successfully.

☕ Starting 5-minute break...
   ☕ Running: 00:01
```

#### View Project Statistics
```bash
netupi> projects
📂 Your Projects:
──────────────────────────────
  📋 Website Redesign (2 hours 30 minutes)
  📋 Mobile App (45 minutes)
  📋 Client Meeting (1 hours 15 minutes)

💡 Use 'work <project-name>' to start tracking time for a project
```

### 📁 Data Storage

All your data is stored locally and securely:

- **Linux/macOS**: `~/.local/share/netupi23/`
- **Windows**: `%LOCALAPPDATA%\netupi23\`

Files stored:
- `sessions.json`: All your work sessions with timestamps
- `config.json`: Application configuration and preferences

### 🛠️ Development Status

Netupi23 is actively being developed. Current focus areas:

**Implemented:**
- ✅ Basic Pomodoro and break timers
- ✅ Project time tracking
- ✅ Real-time timer display
- ✅ Auto-stop functionality
- ✅ Data persistence

**Coming Soon:**
- [ ] Pause/Resume functionality
- [ ] Daily summary reports  
- [ ] Enhanced timer display options
- [ ] Configuration customization

### 🐛 Known Issues

- Timer display positioning may vary between terminal emulators
- History persistence requires the `with-file-history` feature

### 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### 🙏 Acknowledgments

**Special thanks to [4DA](https://github.com/4DA) for the original inspiration and concept that made this project possible.**

- Built with [Rust](https://www.rust-lang.org/) for performance and reliability
- Uses [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal manipulation
---

## Español

**Inspirado y basado en el trabajo de [4DA](https://github.com/4DA)**  
*Todos los reconocimientos a 4DA por el concepto original e inspiración que hizo posible este proyecto.*

Una aplicación de seguimiento de tiempo minimalista escrita en Rust. Perfecta para desarrolladores, freelancers y cualquier persona que quiera rastrear su tiempo de trabajo de manera eficiente.

### ✨ Características Actuales

- **⏰ Temporizador Pomodoro**: Sesiones de trabajo enfocado de 25 minutos
- **☕ Temporizador de Descanso**: Sesiones de descanso de 5 minutos
- **📊 Seguimiento de Tiempo por Proyecto**: Rastrea tiempo en diferentes proyectos con almacenamiento persistente
- **💾 Almacenamiento Local**: Todas las sesiones guardadas localmente en formato JSON
- **🎯 Visualización de Temporizador en Tiempo Real**: Ve el progreso de tu temporizador actualizándose en vivo en el terminal
- **🚀 Modo Interactivo**: Interfaz de línea de comandos intuitiva con historial y auto-completado
- **⚡ Función de Auto-parada**: Iniciar un nuevo temporizador detiene automáticamente el actual
- **🔒 Privacidad Primero**: Todos los datos se quedan en tu computador - sin nube, sin seguimiento

### 🚀 Inicio Rápido

```bash
# Clonar el repositorio
git clone https://github.com/gatzer/netupi23.git
cd netupi23

# Compilar y ejecutar
cargo run
```

### 📋 Comandos Disponibles

| Comando | Alias | Descripción |
|---------|-------|-------------|
| `work <nombre-proyecto>` | `w` | Iniciar seguimiento de tiempo para un proyecto específico |
| `pomodoro` | `pomo` | Iniciar una sesión de trabajo Pomodoro de 25 minutos |
| `break` | `br` | Iniciar un temporizador de descanso de 5 minutos |
| `stop` | `s` | Detener el temporizador actual y guardar la sesión |
| `projects` | `proj` | Listar todos los proyectos con tiempo total trabajado |
| `help` | `h`, `?` | Mostrar comandos disponibles |
| `clear` | `cls` | Limpiar la pantalla del terminal |
| `exit` | `quit`, `q` | Salir de la aplicación |

### 💡 Ejemplos de Uso

#### Rastrear Tiempo de Trabajo
```bash
netupi> work "Rediseño Web"
🚀 Iniciando sesión de trabajo para proyecto: Rediseño Web
⏰ ¡Temporizador de trabajo iniciado! Usa 'stop' para terminar.
   ⚡ Ejecutándose: 00:15
netupi> stop
✅ ¡Temporizador detenido!
⏱️  Tiempo total: 15 minutos
💾 Sesión guardada exitosamente.
```

#### Flujo de Trabajo Pomodoro
```bash
netupi> pomodoro
🍅 Iniciando sesión Pomodoro de 25 minutos...
⏰ ¡Pomodoro iniciado! Usa 'stop' para terminar antes.
   🍅 Ejecutándose: 00:05

# El temporizador se actualiza en tiempo real mientras trabajas...
```

#### Función de Auto-parada
```bash
netupi> pomodoro
🍅 Iniciando sesión Pomodoro de 25 minutos...
   🍅 Ejecutándose: 00:05

netupi> break
⏹️  ¡Temporizador anterior detenido antes de tiempo!
⏱️  Tiempo registrado: 5 minutos
💾 Sesión guardada exitosamente.

☕ Iniciando descanso de 5 minutos...
   ☕ Ejecutándose: 00:01
```

#### Ver Estadísticas de Proyecto
```bash
netupi> projects
📂 Tus Proyectos:
──────────────────────────────
  📋 Rediseño Web (2 horas 30 minutos)
  📋 App Móvil (45 minutos)
  📋 Reunión Cliente (1 horas 15 minutos)

💡 Usa 'work <nombre-proyecto>' para empezar a rastrear tiempo en un proyecto
```

### 📁 Almacenamiento de Datos

Todos tus datos se almacenan local y seguramente:

- **Linux/macOS**: `~/.local/share/netupi23/`
- **Windows**: `%LOCALAPPDATA%\netupi23\`

Archivos almacenados:
- `sessions.json`: Todas tus sesiones de trabajo con timestamps
- `config.json`: Configuración de la aplicación y preferencias

### 🛠️ Estado de Desarrollo

Netupi23 está siendo desarrollado activamente. Áreas de enfoque actuales:

**Implementado:**
- ✅ Temporizadores básicos de Pomodoro y descanso
- ✅ Seguimiento de tiempo por proyecto
- ✅ Visualización de temporizador en tiempo real
- ✅ Funcionalidad de auto-parada
- ✅ Persistencia de datos

**Próximamente:**
- [ ] Funcionalidad de Pausar/Reanudar
- [ ] Reportes de resumen diario
- [ ] Opciones mejoradas de visualización de temporizador
- [ ] Personalización de configuración

### 🐛 Problemas Conocidos

- El posicionamiento de la visualización del temporizador puede variar entre emuladores de terminal
- La persistencia del historial requiere la característica `with-file-history`

### 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

### 🙏 Agradecimientos

**Agradecimientos especiales a [4DA](https://github.com/4DA) por la inspiración original y el concepto que hizo posible este proyecto.**

- Construido con [Rust](https://www.rust-lang.org/) para rendimiento y confiabilidad
- Usa [crossterm](https://github.com/crossterm-rs/crossterm) para manipulación de terminal multiplataforma

---

# Git Repo Analyzer

Herramienta que analiza repositorios públicos de GitHub y genera un reporte detallado sobre su estructura, tecnologías, calidad del código y un resumen generado por IA.

## ¿Qué hace?

Dado una URL de GitHub, el backend:
- Clona el repositorio temporalmente
- Analiza su estructura, archivos y dependencias
- Detecta tecnologías, lenguaje dominante, licencia y estado general
- Busca code smells (archivos grandes, TODOs, configuraciones expuestas)
- Llama a Gemini para generar un resumen, clasificación y sugerencias de mejora
- Limpia el repositorio clonado al finalizar

## Stack

- **Rust** + **Axum** — servidor HTTP
- **Tokio** — runtime async
- **Reqwest** — llamadas a la API de Gemini
- **Gemini 2.5 Flash Lite** — análisis con IA

## Requisitos

- Rust + Cargo
- Git CLI disponible en el PATH
- API Key de [Google AI Studio](https://aistudio.google.com)

## Configuración

Crea un archivo `.env` en la raíz del proyecto:

```env
GEMINI_API_KEY=tu_api_key_aqui
```

## Correr el servidor

```bash
cargo run
```

El servidor queda disponible en `http://localhost:3000`.

---

## API

### `GET /health`

Verifica que el servidor esté corriendo.

**Respuesta**
```
OK
```

---

### `POST /api/analyze`

Analiza un repositorio público de GitHub.

**Body**
```json
{
  "repo_url": "https://github.com/usuario/repositorio"
}
```

**Respuesta**
```json
{
  "repo_url": "https://github.com/usuario/repositorio",
  "technologies": ["Rust", "Docker"],
  "dominant_language": "Rust",
  "directory_tree": ["src/", "  main.rs", "Cargo.toml"],
  "total_files": 30,
  "total_lines": 1485,
  "dependency_count": 4,
  "has_readme": true,
  "has_tests": false,
  "has_license": true,
  "license_type": "MIT",
  "last_commit_days": 23,
  "contributors": 1,
  "score": 8,
  "code_smells": [
    {
      "kind": "large_file",
      "location": "src/main.rs",
      "detail": "Archivo con 450 líneas. Considerar dividirlo en módulos más pequeños."
    }
  ],
  "ai": {
    "summary": "Descripción del proyecto...",
    "complexity": "high",
    "category": "tool",
    "difficulty": "advanced",
    "suggestions": [
      "Agregar tests unitarios.",
      "Añadir una licencia."
    ]
  }
}
```

**Campos de `ai`**

| Campo | Valores posibles |
|---|---|
| `complexity` | `low` `medium` `high` |
| `category` | `cli` `api` `library` `web-app` `game` `tool` `other` |
| `difficulty` | `beginner` `intermediate` `advanced` |

**Errores**

| Código | Motivo |
|---|---|
| `400` | URL inválida, no es de GitHub, o repo no encontrado |
| `429` | Límite de requests alcanzado (5 por cada 10 minutos) |

---

### `POST /api/compare`

Compara dos repositorios públicos de GitHub y devuelve un veredicto generado por IA.

**Body**
```json
{
  "repo_a": "https://github.com/usuario/repo-uno",
  "repo_b": "https://github.com/usuario/repo-dos"
}
```

**Respuesta**

Devuelve el análisis completo de cada repo más un objeto `comparison`:

```json
{
  "repo_a": { ... },
  "repo_b": { ... },
  "comparison": {
    "verdict": "repo_b",
    "reason": "El repositorio B tiene mayor madurez...",
    "repo_a_strengths": ["Bien estructurado", "Usa Rust"],
    "repo_b_strengths": ["Tiene tests", "Licencia MIT", "CI/CD configurado"],
    "recommendation": "Continuar desarrollando repo_a añadiendo tests..."
  }
}
```

**Campos de `comparison`**

| Campo | Descripción |
|---|---|
| `verdict` | Cuál repo está mejor: `repo_a`, `repo_b` o `tie` |
| `reason` | Justificación del veredicto |
| `repo_a_strengths` | Fortalezas específicas del repo A |
| `repo_b_strengths` | Fortalezas específicas del repo B |
| `recommendation` | Qué hacer con cada repositorio |

**Nota:** comparar cuenta como 2 requests para el rate limiter.

---

## Rate Limiting

Cada IP tiene un límite de **5 requests cada 10 minutos**. Al superarlo se devuelve un `429`:

```json
{
  "error": "Demasiadas solicitudes. Has alcanzado el límite. Solicitudes restantes: 0. Intenta de nuevo en unos minutos."
}
```
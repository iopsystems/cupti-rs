# Documentation Conversion Guide

This document describes the rules for converting C API documentation from CUPTI headers to Rust rustdoc format.

## Documentation Conversion Rules

### 1. Structure
- Use `///` for doc comments on public items
- Start with a brief one-line summary
- If the paragraph after the `\brief` just repeats the brief, remove it
- Organize content into standard rustdoc sections in this order:
  1. Brief description
  2. Additional details (if any)
  3. `# Parameters` (if applicable)
  4. `# Notes` (if applicable)
  5. `# Errors` (for fallible functions)

### 2. Parameters Section
- Only document parameters that exist on the Rust method signature
- Omit parameters like `subscriber` when it becomes `&self`
- Use backticks for parameter names: `` `enable` ``
- Convert C conventions to Rust equivalents:
  - "Zero disables, non-zero enables" → "`false` disables, `true` enables"
  - Pointer/output parameters → describe the return value instead

### 3. Error Documentation
- Use `Error::<Variant>` format for error links, NOT `CUptiResult::<Variant>`
- Drop the `Error` prefix from variant names: `Error::NotInitialized` not `Error::ErrorNotInitialized`
- List all possible error returns from the C API documentation

### 4. Notes Section
- Include important behavioral details
- **Remove thread-safety notes** if the Rust wrapper handles synchronization internally (e.g., with locks)
- Keep domain-specific limitations (e.g., "Names are available only for DRIVER and RUNTIME domains")

### 5. Cross-References
- Use markdown link syntax: `` [`Error::Variant`] `` for types and errors
- Use `` [`method_name`] `` for method references
- Add explicit link targets when needed: `` [`enable_callback`]: Self::enable_callback ``

### 6. Language Conversions
- Replace C-style descriptions with Rust idioms
- `NULL` → `None`
- Return value descriptions → integrate into the brief or return type documentation
- "Returns non-zero if..." → "Returns `true` if..."

### 7. Enum Naming Conventions
- Use PascalCase for enum variant names (not SCREAMING_SNAKE_CASE)
- Strip the common prefix from C enum names (e.g., `CUPTI_ACTIVITY_OBJECT_PROCESS` → `Process`)
- Omit the `FORCE_INT` sentinel values used in C for ABI compatibility

## Example

### C API Documentation
```c
/**
 * \brief Get the current enabled/disabled state of a callback for a specific
 * domain and function ID.
 *
 * Returns non-zero in \p *enable if the callback for a domain and
 * callback ID is enabled, and zero if not enabled.
 *
 * \note \b Thread-safety: a subscriber must serialize access to
 * cuptiGetCallbackState, cuptiEnableCallback, cuptiEnableDomain, and
 * cuptiEnableAllDomains.
 *
 * \param enable Returns non-zero if callback enabled, zero if not enabled
 * \param subscriber Handle to the initialize subscriber
 * \param domain The domain of the callback
 * \param cbid The ID of the callback
 *
 * \retval CUPTI_SUCCESS on success
 * \retval CUPTI_ERROR_NOT_INITIALIZED if unable to initialized CUPTI
 * \retval CUPTI_ERROR_INVALID_PARAMETER if \p enabled is NULL, or if \p
 * subscriber, \p domain or \p cbid is invalid.
 */
CUptiResult CUPTIAPI cuptiGetCallbackState(uint32_t *enable,
                                           CUpti_SubscriberHandle subscriber,
                                           CUpti_CallbackDomain domain,
                                           CUpti_CallbackId cbid);
```

### Rust Documentation
```rust
/// Get the current enabled/disabled state of a callback for a specific domain and function ID.
///
/// Returns `true` if the callback for a domain and callback ID is enabled, and `false` if not
/// enabled.
///
/// # Parameters
///
/// - `domain`: The domain of the callback
/// - `cbid`: The ID of the callback
///
/// # Errors
///
/// - [`Error::NotInitialized`] if unable to initialize CUPTI
/// - [`Error::InvalidParameter`] if `domain` or `cbid` is invalid
pub fn get_callback_state(&self, domain: CallbackDomain, cbid: CallbackId) -> Result<bool> {
    // implementation
}
```

Note how:
- The `enable` output parameter became the return value
- The `subscriber` parameter became `&self`
- Thread-safety notes were removed (handled by internal lock)
- Error variants use `Error::` prefix without repeating `Error` in the variant name
- Boolean convention changed from "zero/non-zero" to "`false`/`true`"

# Contributing to Mayon C Library

Thank you for your interest in contributing to **Mayon**! Mayon is a high-performance UI renderer built on top of **Vulkan**, designed to be consumed via a stable **C API**.

> **Important:** Mayon is **implemented in Rust**. All public API documentation is written in **Rust doc comments** and then exported to C using `cbindgen`. Even though the final output is a C header (`mayon.h`), contributors must follow Rust documentation rules.

---

## 🛠 Documentation Guidelines (CRITICAL)

Because Mayon exports a C API via `cbindgen`, we follow **strict documentation rules**. Our automated tools (including CodeRabbit) and the build pipeline rely on these rules to generate clean, correct, and IDE-friendly C headers.

### The Triple-Slash Rule

Always use `///` for documentation comments.

These comments are parsed by **Rust** and consumed by `cbindgen`, which converts them into `/** ... */` blocks in the generated `mayon.h` file.

❌ Do **not** use:

- `//` for documentation
- `/* ... */` for API docs

✅ Always use:

- `///`

### Doxygen Style (for C Consumers)

Although the code is written in Rust, the documentation must follow **Doxygen-style conventions** so that C developers get proper IntelliSense and documentation in their IDEs.

Supported and required tags include:

- `@brief` — Short description of the item
- `@param` — Description of a function parameter
- `@return` — Description of the return value
- `@note` — Important usage details or constraints

---

### Example (Rust Code Exported to C)

```rust
/// @brief Creates a new Vulkan-backed UI surface.
/// @param instance The active Vulkan instance handle.
/// @return A handle to the created surface, or NULL on failure.
#[no_mangle]
pub extern "C" fn mayon_create_surface(instance: VkInstance) -> *mut Surface {
    // implementation
}
```

This Rust documentation will be emitted into the C header roughly as:

```c
/**
 * @brief Creates a new Vulkan-backed UI surface.
 * @param instance The active Vulkan instance handle.
 * @return A handle to the created surface, or NULL on failure.
 */
Surface* mayon_create_surface(VkInstance instance);
```

---

## ⚠️ Safety and Error Reporting

Mayon’s C API is designed to be **explicit, defensive, and predictable**. When contributing new APIs, please document **safety requirements** and **error behavior** clearly.

### Safety Notes

If a function has any safety constraints, they **must be documented** using Doxygen-style notes. This is especially important for C consumers, who do not benefit from Rust’s safety guarantees.

Use `@note` to describe:

- Ownership rules (who allocates / who frees)
- Lifetime requirements
- Thread-safety expectations
- Whether NULL pointers are allowed
- Any required initialization or call order

**Example:**

```rust
/// @brief Destroys a Mayon surface.
/// @param surface The surface to destroy.
/// @note The surface must not be used after this call.
/// @note This function is not thread-safe.
#[no_mangle]
pub extern "C" fn mayon_surface_destroy(surface: *mut MynSurface) {
    // implementation
}
```

### Error Possibilities

Whenever possible, contributors should:

- Document **what can go wrong**
- Specify **how failure is reported**

Typical patterns include:

- Returning `NULL` on failure
- Returning a boolean or result code
- Using an explicit error retrieval API

These behaviors must be described using `@return` and/or `@note`.

### Retrieving Errors

If an API can fail, contributors should ensure errors are **observable from C**.

Common approaches in Mayon include:

- Thread-local last-error storage
- Instance-associated error state
- Explicit error handles or error codes

When adding or modifying such APIs, document:

- How the error is stored
- How the caller retrieves it
- Whether retrieving the error clears it

**Example:**

```rust
/// @brief Returns the last error message for the calling thread.
/// @return A null-terminated UTF-8 string describing the last error, or NULL if no error occurred.
/// @note The returned pointer is valid until the next Mayon API call on the same thread.
#[no_mangle]
pub extern "C" fn mayon_last_error_message() -> *const c_char {
    // implementation
}
```

Clear error documentation is mandatory for any API that can fail.

For error implementation check these source files:

- `crates\c_api\src\fallible_result.rs`
- `crates\c_api\src\errors.rs`

---

## ✅ Summary

- Mayon is written in **Rust**, not C
- The public API is exported to C via `cbindgen`
- All API documentation **must** use `///`
- Documentation **must** use Doxygen-style tags
- These rules are **non-negotiable** and enforced by tooling

Following these guidelines ensures Mayon remains pleasant to use from both Rust and C ecosystems.

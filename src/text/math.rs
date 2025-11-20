//! Mathematical notation parsing and rendering
//!
//! Supports a subset of LaTeX-like syntax for scientific notation:
//! - Superscripts: x^2 or x^{10}
//! - Subscripts: `x_i` or x_{max}
//! - Greek letters: \alpha, \beta, \gamma, etc.
//! - Combined: `E_k` = \frac{1}{2}mv^2 (simplified rendering)

use std::collections::HashMap;

/// Mathematical notation element
#[derive(Debug, Clone, PartialEq)]
pub enum MathElement {
    /// Regular text
    Text(String),
    /// Superscript
    Superscript(String),
    /// Subscript
    Subscript(String),
    /// Greek letter
    Greek(String),
}

/// Parsed mathematical notation
#[derive(Debug, Clone)]
pub struct MathNotation {
    elements: Vec<MathElement>,
}

impl MathNotation {
    /// Create empty notation
    #[must_use]
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Add an element
    pub fn add(&mut self, element: MathElement) {
        self.elements.push(element);
    }

    /// Get elements
    #[must_use]
    pub fn elements(&self) -> &[MathElement] {
        &self.elements
    }

    /// Convert to plain text (for non-math rendering backends)
    #[must_use]
    pub fn to_plain_text(&self) -> String {
        let mut result = String::new();
        for element in &self.elements {
            match element {
                MathElement::Text(s) => result.push_str(s),
                MathElement::Superscript(s) => {
                    result.push('^');
                    result.push_str(s);
                }
                MathElement::Subscript(s) => {
                    result.push('_');
                    result.push_str(s);
                }
                MathElement::Greek(s) => {
                    result.push_str(s);
                }
            }
        }
        result
    }
}

impl Default for MathNotation {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse mathematical notation from a string
///
/// Supports:
/// - Superscripts: `x^2` or `x^{10}`
/// - Subscripts: `x_i` or `x_{max}`
/// - Greek letters: `\alpha`, `\beta`, etc.
///
/// # Examples
///
/// ```
/// use velociplot::text::parse_math;
///
/// // Simple superscript
/// let notation = parse_math("x^2");
/// assert_eq!(notation.to_plain_text(), "x^2");
///
/// // Subscript
/// let notation = parse_math("x_i");
/// assert_eq!(notation.to_plain_text(), "x_i");
///
/// // Greek letters
/// let notation = parse_math("\\alpha + \\beta");
/// assert_eq!(notation.to_plain_text(), "α + β");
/// ```
#[must_use]
pub fn parse_math(input: &str) -> MathNotation {
    let mut notation = MathNotation::new();
    let greek_map = create_greek_map();

    let mut chars = input.chars().peekable();
    let mut current_text = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            // Superscript
            '^' => {
                // Save any accumulated text
                if !current_text.is_empty() {
                    notation.add(MathElement::Text(current_text.clone()));
                    current_text.clear();
                }

                // Parse superscript content
                let content = if chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    parse_until(&mut chars, '}')
                } else {
                    chars.next().map(|c| c.to_string()).unwrap_or_default()
                };
                notation.add(MathElement::Superscript(content));
            }

            // Subscript
            '_' => {
                // Save any accumulated text
                if !current_text.is_empty() {
                    notation.add(MathElement::Text(current_text.clone()));
                    current_text.clear();
                }

                // Parse subscript content
                let content = if chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    parse_until(&mut chars, '}')
                } else {
                    chars.next().map(|c| c.to_string()).unwrap_or_default()
                };
                notation.add(MathElement::Subscript(content));
            }

            // Greek letter or command
            '\\' => {
                // Save any accumulated text
                if !current_text.is_empty() {
                    notation.add(MathElement::Text(current_text.clone()));
                    current_text.clear();
                }

                // Parse command name
                let mut command = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphabetic() {
                        command.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Convert to Greek letter if recognized
                if let Some(greek) = greek_map.get(command.as_str()) {
                    notation.add(MathElement::Greek((*greek).to_string()));
                } else {
                    // Unknown command, keep as text
                    current_text.push('\\');
                    current_text.push_str(&command);
                }
            }

            // Regular character
            _ => {
                current_text.push(ch);
            }
        }
    }

    // Add any remaining text
    if !current_text.is_empty() {
        notation.add(MathElement::Text(current_text));
    }

    notation
}

/// Parse characters until delimiter is found
fn parse_until<I>(chars: &mut std::iter::Peekable<I>, delimiter: char) -> String
where
    I: Iterator<Item = char>,
{
    let mut result = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == delimiter {
            chars.next(); // consume delimiter
            break;
        }
        result.push(ch);
        chars.next();
    }
    result
}

/// Create Greek letter mapping
fn create_greek_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Lowercase Greek letters
    map.insert("alpha", "α");
    map.insert("beta", "β");
    map.insert("gamma", "γ");
    map.insert("delta", "δ");
    map.insert("epsilon", "ε");
    map.insert("zeta", "ζ");
    map.insert("eta", "η");
    map.insert("theta", "θ");
    map.insert("iota", "ι");
    map.insert("kappa", "κ");
    map.insert("lambda", "λ");
    map.insert("mu", "μ");
    map.insert("nu", "ν");
    map.insert("xi", "ξ");
    map.insert("omicron", "ο");
    map.insert("pi", "π");
    map.insert("rho", "ρ");
    map.insert("sigma", "σ");
    map.insert("tau", "τ");
    map.insert("upsilon", "υ");
    map.insert("phi", "φ");
    map.insert("chi", "χ");
    map.insert("psi", "ψ");
    map.insert("omega", "ω");

    // Uppercase Greek letters
    map.insert("Alpha", "Α");
    map.insert("Beta", "Β");
    map.insert("Gamma", "Γ");
    map.insert("Delta", "Δ");
    map.insert("Epsilon", "Ε");
    map.insert("Zeta", "Ζ");
    map.insert("Eta", "Η");
    map.insert("Theta", "Θ");
    map.insert("Iota", "Ι");
    map.insert("Kappa", "Κ");
    map.insert("Lambda", "Λ");
    map.insert("Mu", "Μ");
    map.insert("Nu", "Ν");
    map.insert("Xi", "Ξ");
    map.insert("Omicron", "Ο");
    map.insert("Pi", "Π");
    map.insert("Rho", "Ρ");
    map.insert("Sigma", "Σ");
    map.insert("Tau", "Τ");
    map.insert("Upsilon", "Υ");
    map.insert("Phi", "Φ");
    map.insert("Chi", "Χ");
    map.insert("Psi", "Ψ");
    map.insert("Omega", "Ω");

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_text() {
        let notation = parse_math("hello world");
        assert_eq!(notation.to_plain_text(), "hello world");
        assert_eq!(notation.elements().len(), 1);
    }

    #[test]
    fn test_superscript_single() {
        let notation = parse_math("x^2");
        assert_eq!(notation.to_plain_text(), "x^2");
        assert_eq!(notation.elements().len(), 2);
        assert!(matches!(notation.elements()[0], MathElement::Text(_)));
        assert!(matches!(
            notation.elements()[1],
            MathElement::Superscript(_)
        ));
    }

    #[test]
    fn test_superscript_braces() {
        let notation = parse_math("x^{10}");
        assert_eq!(notation.to_plain_text(), "x^10");
        assert_eq!(notation.elements().len(), 2);
    }

    #[test]
    fn test_subscript() {
        let notation = parse_math("x_i");
        assert_eq!(notation.to_plain_text(), "x_i");
        assert_eq!(notation.elements().len(), 2);
        assert!(matches!(notation.elements()[1], MathElement::Subscript(_)));
    }

    #[test]
    fn test_greek_letters() {
        let notation = parse_math("\\alpha + \\beta");
        assert_eq!(notation.to_plain_text(), "α + β");
    }

    #[test]
    fn test_combined() {
        let notation = parse_math("E_k = \\frac{1}{2}mv^2");
        // Note: \frac is not supported, so it appears as text
        let plain = notation.to_plain_text();
        assert!(plain.contains("E_k"));
        assert!(plain.contains("mv^2"));
    }

    #[test]
    fn test_uppercase_greek() {
        let notation = parse_math("\\Delta x");
        assert!(notation.to_plain_text().contains("Δ"));
    }
}

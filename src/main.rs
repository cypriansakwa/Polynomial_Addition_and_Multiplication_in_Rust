fn main() {
    // Define polynomials P(x) = 5x^2 - 4x + 2 and Q(x) = x^3 - 2x^2 + 5
    let p = vec![2,-4,5];  // Coefficients of P(x): 2 + (-4)x + 5x^2
    let q = vec![5, 0, -2, 1]; // Coefficients of Q(x): 5 + (-2)x^2 + 1x^3
    // Add the polynomials
    let sum = add_polynomials(&p, &q);
    let product = multiply_polynomials(&p, &q);
    
    // Calculate degrees
    let deg_sum = degree(&sum);
    let deg_product = degree(&product);
    
    // Print the results
    println!("P(x) + Q(x) = {:?}", sum);
    println!("P(x) * Q(x) = {:?}", product);
    println!("deg(P + Q) = {}", deg_sum);
    println!("deg(P * Q) = {}", deg_product);
}

// Function to add two polynomials
fn add_polynomials(p: &Vec<i32>, q: &Vec<i32>) -> Vec<i32> {
    let max_len = std::cmp::max(p.len(), q.len());
    let mut result = vec![0; max_len];

    for i in 0..p.len() {
        result[i] += p[i];
    }

    for i in 0..q.len() {
        result[i] += q[i];
    }

    result
}

// Function to multiply two polynomials
fn multiply_polynomials(p: &Vec<i32>, q: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; p.len() + q.len() - 1];

    for (i, &p_coeff) in p.iter().enumerate() {
        for (j, &q_coeff) in q.iter().enumerate() {
            result[i + j] += p_coeff * q_coeff;
        }
    }

    result
}

// Function to find the degree of a polynomial
fn degree(poly: &Vec<i32>) -> usize {
    // The degree is the index of the last non-zero coefficient
    for i in (0..poly.len()).rev() {
        if poly[i] != 0 {
            return i;
        }
    }
    0 // Degree of zero polynomial is conventionally 0
}


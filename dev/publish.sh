crates=("serde-doc")

for crate in "${crates[@]}"; do
    echo "Publishing $crate..."
    cargo publish -p "$crate"
    sleep 5 # Wait for a few seconds to avoid rate limiting
done

echo "All crates published successfully."
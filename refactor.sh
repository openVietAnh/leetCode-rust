for file in ./src/solutions/*.rs; do
    # Check if file exists and is readable
    if [ -f "$file" ] && [ -r "$file" ]; then
        # Delete main() function
        sed -i '/fn main()/,/}/d' "$file"
        echo "Deleted main() function in $file"
    else
        echo "Error: Unable to read $file"
    fi
done

echo "Deleted main() functions in all Rust files within ./src/solutions"
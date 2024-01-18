import sys
import pandas as pd
from faker import Faker
import random
from datetime import datetime

def generate_csv_data(num_rows):
    fake = Faker()

    data = {
        "name": [fake.name() for _ in range(num_rows)],
        "email": [fake.email() for _ in range(num_rows)],
        "date_of_birth": [fake.date_of_birth(minimum_age=18, maximum_age=70) for _ in range(num_rows)],
        "score": [random.randint(0, 100) for _ in range(num_rows)],
        "city": [fake.city() for _ in range(num_rows)]
    }

    df = pd.DataFrame(data)

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    csv_filename = f"data/{timestamp}_{num_rows}_data.csv"
    df.to_csv(csv_filename, index=False)

    print(f"CSV file generated: {csv_filename}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script_name.py <number_of_rows>")
        sys.exit(1)
    
    try:
        num_rows = int(sys.argv[1])
    except ValueError:
        print("Please provide an integer for the number of rows.")
        sys.exit(1)

    generate_csv_data(num_rows)

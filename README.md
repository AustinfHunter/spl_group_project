### Setting Up MySQL and .env File

1. **MySQL Setup:**
   - Ensure MySQL is installed and running on your machine.
   - Use the provided SQL script to set up the database schema and tables.

2. **Create .env file:**
   - In the root directory of your Rust project, create a `.env` file.
   - Add the following line to the `.env` file, replacing `[url to mysql db on your machine]` with your actual MySQL connection URL:
     ```
     DATABASE_URL=mysql://username:password@localhost:port/database_name
     ```
   - Replace `username`, `password`, `localhost`, `port`, and `database_name` with your MySQL credentials and database information.

### Running the Rust Program

3. **Cargo Run Setup:**
   - Open your terminal or command prompt.
   - Navigate to the root directory of your Rust project.
   - Run the Rust program with the setup command:
     ```bash
     cargo run setup
     ```
   - When prompted, provide the path to the CSV file that contains the data you want to insert into the database.

4. **Running the Program Without Setup:**
   - After the initial setup, simply run the program without the setup flag:
     ```bash
     cargo run
     ```
     This will execute the program without reinserting all the data from the CSV file.

### Additional Notes:

- Ensure that the `.env` file is properly configured with the correct `DATABASE_URL`.
- After the initial setup, running `cargo run setup` again with the CSV path will attempt to reinsert all the data, so use `cargo run` for normal program execution.
- Always verify the CSV path provided matches the location of the CSV file you want to use.
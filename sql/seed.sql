-- Create 10 cities
DO $$
BEGIN
  FOR i IN 1..10 LOOP
    INSERT INTO cities (city_name) VALUES ('City ' || i);
  END LOOP;
END $$;

-- Create 100 companies and associate each with a random city
DO $$
DECLARE
  city_count INTEGER;
  city_id INTEGER;
BEGIN
  SELECT COUNT(*) FROM cities INTO city_count;

  FOR i IN 1..100 LOOP
    city_id := floor(random() * city_count) + 1;
    INSERT INTO companies (company_name, city_id) VALUES ('Company ' || i, city_id);
  END LOOP;
END $$;

-- Create 1000 users and associate each with a random company
DO $$
DECLARE
  company_count INTEGER;
  company_id INTEGER;
BEGIN
  SELECT COUNT(*) FROM companies INTO company_count;

  FOR i IN 1..1000 LOOP
    company_id := floor(random() * company_count) + 1;
    INSERT INTO users (username, company_id) VALUES ('User ' || i, company_id);
  END LOOP;
END $$;

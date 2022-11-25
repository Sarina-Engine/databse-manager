CREATE TABLE categories(
	vid serial,
	id integer NOT NULL,
	title_fa text NOT NULL,
	code text NOT NULL,
	parent_cat integer NOT NULL DEFAULT 0,	
	done boolean NOT NULL DEFAULT false,

	PRIMARY KEY (vid)
);

CREATE TABLE products(
	vid serial NOT NULL,
	id integer NOT NULL,
	cat_id integer NOT NULL,
	title_fa text NOT NULL,
	rate float NOT NULL,
	views integer NOT NULL,
	done boolean NOT NULL DEFAULT false,

	PRIMARY KEY (vid)
	/*
	FOREIGN KEY (cat_id) REFERENCES categories (id)
	*/
);

CREATE TABLE comments(
	vid serial NOT NULL,
	id integer NOT NULL,
	product_id integer NOT NULL,
	body text NOT NULL,
	rate float NOT NULL,
	
	PRIMARY KEY (vid)
	/*
	FOREIGN KEY (product_id) REFERENCES products (id)
	*/
);

CREATE TABLE features(
	vid serial NOT NULL,
	name text NOT NULL,
	value float NOT NULL default 0,
	product_id integer NOT NULL,

	PRIMARY KEY (vid)
	/*
	FOREIGN KEY (product_id) REFERENCES products (id)
	*/
);

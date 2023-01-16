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
	done boolean NOT NULL DEFAULT false,
	
	PRIMARY KEY (vid)
	/*
	FOREIGN KEY (product_id) REFERENCES products (id)
	*/
);

CREATE TABLE features(
	vid serial NOT NULL,
	name text NOT NULL,
	value text NOT NULL,
	product_id integer NOT NULL,

	PRIMARY KEY (vid)
	/*
	FOREIGN KEY (product_id) REFERENCES products (id)
	*/
);

CREATE TABLE sentiments(
	vid serial NOT NULL,
	comment_id integer NOT NULL,
	recommended float NOT NULL,
	not_recommended float NOT NULL,
	no_idea float NOT NULL,
	sad float NOT NULL,
	happy float NOT NULL,
	positive float NOT NULL,
	negative float NOT NULL,
	furious float NOT NULL,
	angry float NOT NULL,
	neutral float NOT NULL,
	happy2 float NOT NULL,
	delighted float NOT NULL,
	done boolean NOT NULL DEFAULT false,

	PRIMARY KEY (vid)
	/*FOREIGN KEY(comment_id) REFERENCE comments(id) */
);

CREATE TABLE scores(
	vid serial,
	product_id integer NOT NULL,
	emotion float NOT NULL DEFAULT 0,
	satisfaction float NOT NULL DEFAULT 0,
	recommended float NOT NULL DEFAULT 0,
	feeling float NOT NULL DEFAULT 0,
	overall float NOT NULL DEFAULT 0,

	PRIMARY KEY(vid)
);

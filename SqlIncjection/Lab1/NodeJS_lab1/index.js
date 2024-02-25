const axios = require('axios')

const url = "https://0ae1001c040e47fb80c83ff2003c0062.web-security-academy.net/filter?category=";

axios.get(url)
	.then(response => {
		if (response.data.includes('cat')) {
			console.log("Success");
		} else {
			console.log("Failed");
		}
	})
	.catch(error => {
		console.log("Error occured");
	});


const url = "https://0ae1001c040e47fb80c83ff2003c0062.web-security-academy.net/filter?category=;"

const response = await fetch(url);

if (response.status === 200) {
	console.error("succes");
} else {
	console.error("Error during get dta:", response.status);
}
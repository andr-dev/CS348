import Typography from "@ui/typography";
import React, { useState, useEffect } from "react";
import axios from 'axios';
import Flex from "@ui/flex";

const Champs: React.FC = () => {
	const [champData, setChampData] = useState({ winrate: [] });

	useEffect(() => {
		const fetchChampData = () => {
			axios("http://localhost:8000/api/champion/winrate?min=0&max=10000").then((result) => {
				console.log(result.data.Ok);
				setChampData({ winrate: result.data.Ok });
			});
		}

		fetchChampData();
	}, []);

	return (
		<Flex column>
			<Typography level="header">Champs</Typography>
			<Flex column>
				{champData.winrate.map((a, b) => <p>{a}</p>)}
			</Flex>
		</Flex>
	)
};

export default Champs;

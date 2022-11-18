import Typography from "@ui/typography";
import React, { useState, useEffect } from "react";
import axios from "axios";
import Flex from "@ui/flex";
import styled from "styled-components";

const Champs: React.FC = () => {
  const timeIntervals = [
    { lo: 0, hi: 15 * 60 },
    { lo: 15 * 60, hi: 30 * 60 },
    { lo: 30 * 60, hi: 180 * 60 },
  ];

  const [champData, setChampData] = useState<any>({
    intervalWinrates: [[], [], []],
  });
  useEffect(() => {
    const fetchChampData = () => {
      Promise.all(
        timeIntervals.map((interval) =>
          axios(
            `http://localhost:8000/api/champion/winrate?min=${interval.lo}&max=${interval.hi}`
          )
        )
      ).then((res) => {
        const tmp1 = res.map((v) => v.data.Ok);
        console.log(tmp1);
        setChampData({
          intervalWinrates: tmp1,
        });
      });
    };

    fetchChampData();
  }, []);

  return (
    <Flex column grow={1}>
      <Typography level="header">Champs</Typography>
      <TableRow>
        <Flex grow={1}>
          <Typography level="button">Champion</Typography>
        </Flex>
        <TableItem>{ }</TableItem>
        <TableItem><Typography level="button">Winrate</Typography></TableItem>
        <TableItem>{ }</TableItem>
      </TableRow>
      <TableRow>
        <Flex grow={1}>{ }</Flex>
        <TableItem><Typography level="button">0-15 min</Typography></TableItem>
        <TableItem><Typography level="button">15-30 min</Typography></TableItem>
        <TableItem><Typography level="button">30+ min</Typography></TableItem>
      </TableRow>
      <Spacer>{ }</Spacer>
      {champData.intervalWinrates[0]?.map((a: any, i: any) => (
        <TableRow align="center">
          <Flex grow={1} align="center">
            <img src={`http://ddragon.leagueoflegends.com/cdn/12.22.1/img/champion/${getChampName(a[0])}.png`} style={{ padding: "8px" }} />
            <Typography level="button">{a[0]}</Typography>
          </Flex>
          <TableItem><Typography level="button">{(a[1] * 100).toFixed(2)}%</Typography></TableItem>
          <TableItem><Typography level="button">{(champData.intervalWinrates[1][i][1] * 100).toFixed(2)}%</Typography></TableItem>
          <TableItem><Typography level="button">{(champData.intervalWinrates[2][i][1] * 100).toFixed(2)}%</Typography></TableItem>
        </TableRow>
      ))
      }
    </Flex >
  );
};

const getChampName = (name: any) => {
  let x = name.replace(/\s/g, '');

  let ai = x.indexOf("'");
  let pi = x.indexOf("\.");

  if (x == "Nunu&Willump") {
    return "Nunu";
  }

  if (ai != -1) {
    return x.substr(0, ai) + x.charAt(ai + 1).toLowerCase() + x.substr(ai + 2);
  } else if (pi != -1) {
    return x.substr(0, pi) + x.substr(pi + 1)
  }

  return x;
}

const Spacer = styled.div`
  height: 8px;
`;

const TableRow = styled(Flex)`
  width: 100%;
`;

const TableItem = styled(Flex)`
  width: 96px;
`;


export default Champs;

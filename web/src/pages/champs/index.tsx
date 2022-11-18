import Typography from "@ui/typography";
import React, { useState, useEffect } from "react";
import axios from "axios";
import Flex from "@ui/flex";

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
    <Flex column>
      <Typography level="header">Champs</Typography>
      <Flex>
        <Typography
          style={{
            width: "100px",
          }}
          level="button"
        >
          Champion
        </Typography>
        <Typography
          style={{
            width: "100px",
          }}
          level="button"
        >
          {`Winrate`}
        </Typography>
      </Flex>
      <Flex>
        <Typography
          style={{
            width: "100px",
            marginLeft: "100px",
            marginRight: "10px",
          }}
          level="button"
        >
          {`0-15 min`}
        </Typography>
        <Typography
          style={{
            width: "100px",
            marginRight: "10px",
          }}
          level="button"
        >
          {`15-30 min`}
        </Typography>
        <Typography
          style={{
            width: "100px",
            marginRight: "10px",
          }}
          level="button"
        >
          {`30+ min`}
        </Typography>
      </Flex>
      {champData.intervalWinrates[0]?.map((a: any, i: any) => (
        <Flex>
          <Typography
            style={{
              width: "100px",
              overflow: "hidden",
            }}
            level="button"
          >{`${a[0]}`}</Typography>
          <Typography
            style={{
              width: "100px",
              overflow: "hidden",
              marginRight: "10px",
            }}
            level="button"
          >{` ${(a[1] * 100).toFixed(2)}%`}</Typography>
          <Typography
            style={{
              width: "100px",
              overflow: "hidden",
              marginRight: "10px",
            }}
            level="button"
          >{` ${(champData.intervalWinrates[1][i][1] * 100).toFixed(
            2
          )}%`}</Typography>
          <Typography
            style={{
              width: "100px",
              overflow: "hidden",
              marginRight: "10px",
            }}
            level="button"
          >{` ${(champData.intervalWinrates[2][i][1] * 100).toFixed(
            2
          )}%`}</Typography>
        </Flex>
      ))}
    </Flex>
  );
};

export default Champs;

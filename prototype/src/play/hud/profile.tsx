import { useEffect, useState } from "preact/hooks"
import { Module } from "./module"
import { receive } from "../../server"
import { addTrailingZero } from "../../utils"

export function Profile() {
  const [data, setData] = useState<{ name: string, seed: number } | undefined>()
  const [time, setTime] = useState<{ day: number, hour: number, minute: number } | undefined>()
  const [health, setHealth] = useState<number | undefined>()
  const [energy, setEnergy] = useState<number | undefined>()
  const [money, setMoney] = useState<{ [type: string]: number }>({})

  useEffect(() => {
    const cancelData = receive("profile", setData)
    const cancelTime = receive("time", setTime)
    const cancelHealth = receive("health", setHealth)
    const cancelEnergy = receive("energy", setEnergy)
    const cancelMoney = receive("money", setMoney)
    return () => {
      cancelData()
      cancelTime()
      cancelHealth()
      cancelEnergy()
      cancelMoney()
    }
  }, [])

  if(!data) return null

  const moneyList = Object.keys(money)

  return (
    <Module>
      <strong>Profile</strong>
      <span>Name: {data.name}</span>
      <span>Seed: {data.seed}</span>
      {time && <span>
        Time: day {time.day} - {addTrailingZero(time.hour)}:{addTrailingZero(time.minute)} {
          (time.hour >= 6 && time.hour < 18) ? "☀️" : "🌙"
        }
      </span>}
      {typeof health !== "undefined" && <span>Health: <ProgressBar value={health}/></span>}
      {typeof energy !== "undefined" && <span>Energy: <ProgressBar value={energy}/></span>}
      {moneyList.length > 0 && (
        <span>
          Money: {moneyList.map((type) => (money[type] + " " + type)).join(", ")}
        </span>
      )}
    </Module>
  )
}

function ProgressBar(props: { value: number }) {
  return (
    <span
      style={{
        width: "100%",
        height: "1em",
        backgroundColor: "black",
        marginLeft: "0.5em",
      }}
      title={props.value + "%"}
    >
      <span
        style={{
          width: props.value + "%",
          height: "100%",
          backgroundColor: "green",
          display: "block",
        }}
      />
    </span>
  )
}

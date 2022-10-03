
export default function Home() {
	const onClickBtn = () => {
		fetch("http://localhost:1337/hello_world", {
			headers: {
				authorization: "Bearer 12312312312312312321321312321"
			}
		})
		.then((res) => res.json())
		.catch((err) => console.log(err))
	}
  return (
    <div>
			<button onClick={onClickBtn}>Click me</button>
    </div>
  )
}

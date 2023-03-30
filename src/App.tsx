import { createSignal, Signal, JSX, createEffect } from "solid-js"
import logo from "./assets/logo.svg"
import { invoke } from "@tauri-apps/api/tauri"
import "./App.css"
import { fs, path } from "@tauri-apps/api"
type subcategory = { name: string, input: JSX.Element }
type setting = { name: string, subcategories: subcategory[] }
async function Subcategory(
    name: string,
    props:
        | JSX.InputHTMLAttributes<HTMLInputElement>
        | { type: "select"; options: string[] },
    path: string,
    valueName: string,
    function_name: string = "setting",
): Promise<subcategory> {
    const [value, setValue] = createSignal<string>(await invoke<string>(`get_${function_name}`, { path, valueName }))
    createEffect(() => {
        invoke(`set_${function_name}`, { path, valueName, value: value() })
    })
    if (props.type === "select") {
        const selectProps = props as { type: "select"; options: string[] }
        return {
            name: name,
            input: <select value={value()} onChange={(e) => setValue((e.target as HTMLInputElement).value)}>
                {selectProps.options.map((option) => (
                    <option value={option}>{option}</option>
                ))}
            </select>
        }
    }
    const val = props.type == "checkbox" ? (x: HTMLInputElement) => setValue(x.checked ? "1" : "0") : (x: HTMLInputElement) => setValue(x.value)
    return {
        name: name,
        input: <input{...props} checked={props.type == "checkbox" && value() == "1"} value={value()} onChange={(e) => val(e.target as HTMLInputElement)} />
    }

}
const get_settings: () => Promise<setting[]> = async () => {
    let settings_json: {
        name: string,
        subcategories: {
            name: string,
            function_name?: string,
            path?: string,
            value?: string,
            input: JSX.InputHTMLAttributes<HTMLInputElement> | { type: "select"; options: string[] },
        }[]
    }[] = JSON.parse(await fs.readTextFile(await path.resolveResource('resources/settings.json')))
    return await Promise.all(settings_json.map(async (setting) => {
        return {
            name: setting.name,
            subcategories: await Promise.all(setting.subcategories.map(async (subcategory) =>
                await Subcategory(subcategory.name, subcategory.input, subcategory.path ?? '', subcategory.value ?? '', subcategory.function_name)))
        }
    }))

}
console.log(get_settings())
function App() {
    const [settings, setSettings] = createSignal<setting[]>([])
    get_settings().then(setSettings)
    const [selectedSetting, setSelectedSetting] = createSignal<setting | undefined>()

    return (
        <div style={{ display: 'flex' }}>
            <div style={{ width: '200px' }}>
                <h3>Settings</h3>
                <ul>
                    {settings().map(setting => (
                        <li
                            style={{
                                padding: '10px',
                                cursor: 'pointer',
                                "background-color": selectedSetting() === setting ? '#e5e5e5' : 'white',
                            }}
                            onClick={() => setSelectedSetting(setting)}
                        >
                            {setting.name}
                        </li>
                    ))}
                </ul>
            </div>
            <div style={{ flex: 1, padding: '20px' }}>
                <h2>{selectedSetting()?.name ?? ''}</h2>
                <ul>
                    {(selectedSetting()?.subcategories ?? []).map(subcategory => (
                        <li
                            style={{
                                display: "flex",
                                "align-items": "center",
                                "justify-content": "space-between",
                                "margin-bottom": "10px",
                            }}
                        >
                            <span
                                style={{
                                    "font-family": '"Segoe UI", sans-serif',
                                    "font-size": "14px",
                                }}
                            >
                                {subcategory.name}
                            </span>
                            <span>{subcategory.input}</span>
                        </li>
                    ))}
                </ul>
            </div>
        </div>
    )
}

export default App

import { createSignal, For, Setter } from "solid-js"
import { Page, ProjectModel } from "../models";
import { get_projects } from "../api";
const [projects, setProjects] = createSignal<ProjectModel[]>([]);
export function Project() {
    get_projects().then((p: Page<ProjectModel>) => {
        setProjects(p.items);
    })
return (
    <For each={projects()}>
        {(item, index) =>
            <p>{item.name}</p>
        }
    </For>
    )
}

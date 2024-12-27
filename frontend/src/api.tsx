import { ProjectModel, Page } from "./models";

const SESSION_TOKEN = "H/y589KU0HGReXKz6TK9VwpKKtPTjkgBTZdZKR1IIfoPedJLmLFWyAIEcWoWWfUSvOannr81oAqM1TFjOsZqqw==";
export async function get_projects(): Promise<Page<ProjectModel>>{
    return fetch("http://localhost:8080/projects/list", {headers: {Authorization: `Bearer ${SESSION_TOKEN}`}}).then((x) => x.json()).then((j) => j as Page<ProjectModel>)
}


// export async function get_pages<T>(fn:() => Promise<Page<T>>): Promise<Project[]> {

// }
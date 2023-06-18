import {createApp} from 'vue'
import './style.less'
import App from './App.vue'
import axios, {type AxiosResponse} from "axios"

axios.get('/api/json_sample').then((res: AxiosResponse<{ username: string, id: number }>): void => {
    console.log(res.data.id)
    console.log(res.data.username)
    createApp(App).mount('#app')
});


import Router from "express"
import { Request,Response } from "express"


const router = Router()

router.get("/",(req:Request,res:Response)=> {
    return res.send("home")
})
// Retorna o nome de usuario


router.post("/login",(req:Request,res:Response)=> {
    return res.send("login")
})

router.post("/cadastro",(req:Request,res:Response)=> {
    return res.send("cadastro")
})


export {router}


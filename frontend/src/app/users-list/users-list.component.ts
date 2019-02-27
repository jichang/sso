import { Component, OnInit, Input } from "@angular/core";
import { User } from "../model";

@Component({
  selector: "users-list",
  templateUrl: "./users-list.component.html",
  styleUrls: ["./users-list.component.css"]
})
export class UsersListComponent implements OnInit {
  columns: string[] = ["account_name", "role_name", "action"];

  @Input() users: User[];

  constructor() {}

  ngOnInit() {}
}

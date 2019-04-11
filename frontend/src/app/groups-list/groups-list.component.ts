import { Component, OnInit, Input } from "@angular/core";
import { Group } from "../group-model.service";

@Component({
  selector: "groups-list",
  templateUrl: "./groups-list.component.html",
  styleUrls: ["./groups-list.component.css"]
})
export class GroupsListComponent implements OnInit {
  columns: string[] = ["name", "action"];

  @Input() groups: Group[];

  constructor() {}

  ngOnInit() {}
}

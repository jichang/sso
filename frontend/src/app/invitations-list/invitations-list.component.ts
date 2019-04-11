import { Component, OnInit, Input, Output, EventEmitter } from "@angular/core";
import { Invitation } from "../invitation-model.service";

@Component({
  selector: "invitations-list",
  templateUrl: "./invitations-list.component.html",
  styleUrls: ["./invitations-list.component.css"]
})
export class InvitationsListComponent implements OnInit {
  columns: string[] = ["code", "status", "action"];

  @Input() invitations: Invitation[];
  @Output() remove = new EventEmitter();

  constructor() {}

  ngOnInit() {}

  removeInvitation(invitation: Invitation) {
    this.remove.emit(invitation);
  }
}

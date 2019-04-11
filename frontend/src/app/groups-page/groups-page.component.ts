import { Component, OnInit } from "@angular/core";
import { Validators, FormBuilder, FormGroup } from "@angular/forms";
import { session } from "../model";
import { Group, GroupModelService } from "../group-model.service";
import { Subscription } from "rxjs";
import { Router } from "@angular/router";
import { MatSnackBar } from "@angular/material";

@Component({
  selector: "groups-page",
  templateUrl: "./groups-page.component.html",
  styleUrls: ["./groups-page.component.css"]
})
export class GroupsPageComponent implements OnInit {
  groups: Group[] = [];
  groupsSubscription: Subscription;

  constructor(
    private router: Router,
    private groupModel: GroupModelService,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {
    this.groupsSubscription = this.groupModel.groups.subscribe(
      groups => {
        this.groups = groups;
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.groupModel.select();
    } else {
      this.router.navigate(["signin"]);
    }
  }

  ngOnDestroy() {
    this.groupsSubscription.unsubscribe();
  }
}

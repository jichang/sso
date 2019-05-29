import { Component, OnInit, OnDestroy } from "@angular/core";
import { GroupModelService, Group } from "../group-model.service";
import { ActivatedRoute, Router } from "@angular/router";
import { PaginatorParams, User, session } from "../model";
import { MatDialog } from "@angular/material/dialog";
import { PageEvent } from "@angular/material/paginator";
import { MatSnackBar } from "@angular/material/snack-bar";
import { Subscription } from "rxjs";

@Component({
  selector: "group-page",
  templateUrl: "./group-page.component.html",
  styleUrls: ["./group-page.component.css"]
})
export class GroupPageComponent implements OnInit {
  pageSizeOptions: number[] = [5, 10, 25, 100];
  paginatorParams: PaginatorParams = { offset: 0, limit: 20 };
  total: number = 0;
  groups: Group[] = [];
  users: User[] = [];
  displayedColumns: string[] = ["name", "group_name"];
  columns: string[] = ["name", "group_name"];
  groupsSubscription: Subscription;
  usersSubscription: Subscription;

  constructor(
    private groupModel: GroupModelService,
    private router: Router,
    private route: ActivatedRoute,
    private snackBar: MatSnackBar,
    public dialog: MatDialog
  ) {}

  ngOnInit() {
    let groupId = this.route.snapshot.params["id"];
    this.usersSubscription = this.groupModel.users.subscribe(
      usersCache => {
        let groupUsers = usersCache[groupId];
        if (groupUsers) {
          this.total = groupUsers.total;
          let page = groupUsers.pages.find(page => {
            return page.offset === this.paginatorParams.offset;
          });
          if (page) {
            this.users = page.users;
          }
        }
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );
    this.groupModel.selectUsers(groupId, this.paginatorParams);

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

  changePage(page: PageEvent) {
    this.paginatorParams.limit = page.pageSize;
    this.paginatorParams.offset = 0;

    let groupId = this.route.snapshot.params["id"];
    this.groupModel.selectUsers(groupId, this.paginatorParams);
  }
}
